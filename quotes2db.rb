$KCODE='utf-8'
require 'jcode'

require 'rubygems'
require 'dm-core'
require 'ftools'
require 'models/models'

# Check command line args
if ARGV.length < 2 then
    puts "Usage: quotes2db.rb quotes.db file.quotes user.profile..."
    exit 2
end

db_file = ARGV.shift
db = "sqlite3://#{Dir.pwd}/#{db_file}"

puts "Setting up db: #{db}"
DataMapper.setup(:default, db)
User.auto_migrate!
Quote.auto_migrate!
Rating.auto_migrate!

$quote_id_mapping = {}
$linked_quotes = {}
$favourite_quotes = {}

def add_profile(user, profile_file)
    profile_lines = File.readlines(profile_file)

    profile = User.new
    profile.username = user
    profile.password = profile_lines.shift.chomp
    profile.firstname = profile_lines.shift.chomp
    profile.surname = profile_lines.shift.chomp
    last_posted = profile_lines.shift.to_i
    profile_lines.shift # profile[4] is always zero
    favourite_quote_id = profile_lines.shift.chomp

    # Convert zeros to nil
    profile.last_posted = last_posted.zero? ? nil : Time.at(last_posted)

    unless favourite_quote_id == "0"
        $favourite_quotes[user] = favourite_quote_id
    end

    if profile.save
      puts "Inserted profile for #{user}"
    else
      puts "Error saving profile for #{user}"
    end
end

def add_quotes(username, quotes_file)
    line_number = 0
    user = User.first(:username => username)
    raise "Unable to find user #{username}" unless user
    quotes_file.each_line do |line|
        line.chomp!
        line_number += 1

        # Split on pipe, ratings are ignored as they aren't in a normalised
        # format
        quote, poster, posted, ratings, link_id = line.split('|')

        posted = posted.to_i
        posted = posted.zero? ? nil : Time.at(posted)
        link_id = nil if link_id == "0"

        q = Quote.new(
          :quote_body => quote,
          :poster => User.first(:username => poster),
          :created_at => posted,
          #:linked_quote_id => link_id, will require a second pass
          :user => user
        )

        unless q.save
          puts "Error saving quote [#{line_number}]: #{line}"
        end

        old_quote_id = q.user.username + line_number.to_s
        #puts "Inserted quote #{old_quote_id}"
        #require 'pp'
        #pp quote

        # Get the quote back out to add the id to the mapping
        # inserted_quote = $db[:quotes][quote]
        # if inserted_quote.nil?
        #     puts "Unable to get inserted quote"
        #     return
        # end
        $quote_id_mapping[old_quote_id] = q
        unless link_id.nil?
            # Store link for later after all quotes are loaded
            $linked_quotes[q.id] = link_id
        end
    end
end

puts "Processing files"
quote_files = []
ARGV.each do |file_path|
    if file_path =~ /([^\/]+)\.quotes/
      # Defer quotes until all profiles have been created
      quote_files << [$1, file_path]
    elsif file_path =~ /([^\/]+)\.profile/
        add_profile($1, file_path)
    else
        puts "Ignoring unknown file #{file_path}"
    end
end

# Process quote files
quote_files.each do |user, file_path|
  File.open(file_path) do |file|
    puts "Adding quotes for #{user} from #{file_path}"
    add_quotes(user, file)
  end
end

# Now do a pass through the links and mappings to fill in any missing details
puts "Resolving linked quotes"
$linked_quotes.each do |quote_id,old_id|
    quote = Quote.get!(quote_id)

    if quote.update(:parent_quote => $quote_id_mapping[old_id])
      puts "Updated quote #{quote.id}"
    else
      puts "Error updating linked quote for quote #{quote.id}"
    end
end

puts "Resolving favourite quotes"
$favourite_quotes.each do |username, old_id|
    profile = User.first(:username => username)
    if profile.nil?
        puts "Unable to get profile for #{username}"
        exit 4;
    end

    if profile.update(:favourite_quote => $quote_id_mapping[old_id])
      puts "Updated profile #{username}"
    else
      puts "Error updating profile #{username}, with favourite quote"
      debugger
    end
end


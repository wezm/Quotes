$KCODE='utf-8'
require 'jcode'

require 'rubygems'
require 'sequel'
require 'ftools'

# Check command line args
if ARGV.length < 2 then
    puts "Usage: quotes2db.rb quotes.db file.quotes user.profile..."
    exit 2
end

db_file = ARGV.shift

$db = Sequel.sqlite(db_file)
$quote_id_mapping = {}
$linked_quotes = {}
$favourite_quotes = {}

def add_profile(user, profile_file)
    profile_lines = File.readlines(profile_file)

    profile = {}
    profile[:username] = user
    profile[:password] = profile_lines.shift.chomp
    profile[:name] = profile_lines.shift.chomp
    profile[:name] += " " + profile_lines.shift.chomp
    profile[:last_posted] = profile_lines.shift.to_i
    profile_lines.shift # profile[4] is always zero
    favourite_quote_id = profile_lines.shift.chomp

    # Convert zeros to nil
    profile[:last_posted] = nil if profile[:last_posted] == 0

    if favourite_quote_id != "0"
        $favourite_quotes[user] = favourite_quote_id
    end

    $db[:users] << profile
    puts "Inserted profile for #{user}"
end

def add_quotes(user, quotes_file)
    line_number = 0
    quotes_file.each_line do |line|
        line.chomp!
        line_number += 1

        # Split on pipe, ratings are ignored as they aren't in a normalised
        # format
        quote, poster, posted, ratings, link_id = line.split('|')

        posted = posted.to_i 
        posted = nil if posted == 0
        link_id = nil if link_id == "0"

        quote = {
            :quote => quote,
            :poster => poster,
            :posted => posted,
            #:linked_quote_id => link_id, will require a second pass
            :user => user,
        }

        $db[:quotes] << quote
        old_quote_id = quote[:user] + line_number.to_s
        #puts "Inserted quote #{old_quote_id}"
        #require 'pp'
        #pp quote

        # Get the quote back out to add the id to the mapping
        inserted_quote = $db[:quotes][quote]
        if inserted_quote.nil?
            puts "Unable to get inserted quote"
            return
        end
        $quote_id_mapping[old_quote_id] = inserted_quote[:quote_id]
        unless link_id.nil?
            # Store link for later after all quotes are loaded
            $linked_quotes[inserted_quote[:quote_id]] = link_id
        end
    end
end

puts "Processing files"
ARGV.each do |file_path|
    if file_path =~ /([^\/]+)\.quotes/
        File.open(file_path) do |file|
            puts "Adding quotes for #{file_path}"
            add_quotes($1, file)
        end
    elsif file_path =~ /([^\/]+)\.profile/
        add_profile($1, file_path)
    end
end

# Now do a pass through the links and mappings to fill in any missing details
puts "Resolving linked quotes"
$linked_quotes.each do |quote_id,old_id|
    quote = $db[:quotes].filter(:quote_id => quote_id)
    if quote.nil?
        puts "Error getting quote #{quote_id}"
        exit 3;
    end

    quote.update(:linked_quote_id => $quote_id_mapping[old_id])
    puts "Updated quote #{quote_id}"
end

puts "Resolving favourite quotes"
$favourite_quotes.each do |username, old_id|
    profile = $db[:users].filter(:username => username)
    if profile.nil?
        puts "Unable to get profile for #{username}"
        exit 4;
    end

    profile.update(:favourite_quote_id => $quote_id_mapping[old_id])
    puts "Updated profile #{username}, set favourite to #{$quote_id_mapping[old_id]}"
end



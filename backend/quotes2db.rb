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

def add_profile(profile_file)
end

def add_quotes(user, quotes_file)
    line_number = 0
    quotes_file.each_line do |line|
        line.chomp!
        line_number += 1

        # Split on pipe
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
    File.open(file_path) do |file|
        if file_path =~ /([^\/]+)\.quotes/
            puts "Adding quotes for #{file_path}"
            add_quotes($1, file)
        elsif file_path =~ /([^\/]+)\.profile/
            add_profile(file)
        end
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


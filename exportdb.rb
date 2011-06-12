# Exports a quotes db (sqlite) into YAML (suitable for import into a Play!
# based app)

require 'yaml'
require 'dm-core'

%w(user quote rating).each { |model| require "./models/#{model}" }

if ARGV.length < 2 then
    puts "Usage: exportdb.rb quotes.db quotes.yaml"
    exit 2
end

db_file = ARGV.shift
yaml_file = ARGV.shift
db = "sqlite3://#{Dir.pwd}/#{db_file}"

puts "Setting up db: #{db}"
DataMapper.setup(:default, db)
# User.auto_migrate!
# Quote.auto_migrate!
# Rating.auto_migrate!

export = {}

# User(anryan):
#     username:       anryan
#     email:          anryan@example.com
#     passwordHash:   $2a$10$nImReA9wolYvIpvu5S74cOzECiTE6pa2UfWcqueI588ucXii/xByK
#     firstname:      Andrew
#     surname:        Ryan
#     lastPosted:     2003-11-14 13:17:29 +1100
#     isAdmin:        false
#
# Quote(firstWesQuote):
#     body:           Everyone loves to play with Wes' box
#     createdAt:      2011-06-11
#     user:           wmoore
#     poster:         anryan

User.all.each do |user|
  export["User(#{user.username})"] = {
    'username' => user.username,
    'email' => user.email,
    'firstname' => user.firstname,
    'surname' => user.surname,
    'lastPosted' => user.last_posted,
    'isAdmin' => user.username == 'wmoore',
    'favouriteQuote' => user.favourite_quote ? "quote#{user.favourite_quote.id}" : nil
  }
end

Quote.all.each do |quote|
  export["Quote(quote#{quote.id})"] = {
    'body' => quote.quote_body,
    'createdAt' => quote.created_at,
    'poster' => quote.poster.username,
    'parent' => quote.parent_quote ? "quote#{quote.parent_quote.id}" : nil
  }
end

File.open(yaml_file, 'w') do |f|
  YAML.dump(export, f)
end


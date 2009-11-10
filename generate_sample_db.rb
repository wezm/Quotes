require 'rubygems'
require 'dm-core'

%w(user quote rating).each { |model| require "models/#{model}" }

# Check command line args
if ARGV.length < 1 then
    puts "Usage: ruby generate_sample_db.rb sample_quotes.db..."
    exit 2
end

db_file = ARGV.shift
db = "sqlite3://#{Dir.pwd}/#{db_file}"

puts "Setting up db: #{db}"
DataMapper.setup(:default, db)
User.auto_migrate!
Quote.auto_migrate!
Rating.auto_migrate!

users = [
  {
    :firstname => 'Sedrick',
    :surname => 'Block',
    :username => 'sblock',
    :password => 'sblock',
    :password_confirmation => 'sblock',
    :email => 'sblock@example.com'
  },
  {
    :firstname => 'Cortney',
    :surname => 'Kassulke',
    :username => 'ckassulk',
    :password => 'ckassulk',
    :password_confirmation => 'ckassulk',
    :email => 'ckassulk@example.com'
  },
  {
    :firstname => 'Dean',
    :surname => 'Gusikowski',
    :username => 'dgusikow',
    :password => 'dgusikow',
    :password_confirmation => 'dgusikow',
    :email => 'dgusikow@example.com'
  },
  {
    :firstname => 'Jackeline',
    :surname => 'Okuneva',
    :username => 'jokuneva',
    :password => 'jokuneva',
    :password_confirmation => 'jokuneva',
    :email => 'jokuneva@example.com'
  },
]

users.each do |u|
  user = User.new(u)
  user.save || raise("Error creating #{u[:username]}")
end

quotes = [
  {
    :quote_body => "Quia voluptates quis impedit.",
    :user => User.first(:username => 'sblock'),
    :poster => User.first(:username => 'jokuneva')
  },
  {
    :quote_body => "Quisquam id voluptate laboriosam cum mollitia voluptatem impedit distinctio.",
    :user => User.first(:username => 'dgusikow'),
    :poster => User.first(:username => 'sblock')
  },
  {
    :quote_body => "Occaecati ipsa earum maiores assumenda dolorem dolor omnis.",
    :user => User.first(:username => 'jokuneva'),
    :poster => User.first(:username => 'ckassulk')
  },
  {
    :quote_body => "Self quotes aren't cool.",
    :user => User.first(:username => 'dgusikow'),
    :poster => User.first(:username => 'dgusikow')
  },
  {
    :quote_body => "Quisquam ut ipsa sit pariatur ut modi eum reiciendis eaque autem sunt distinctio vero sapiente quo harum est hic voluptas qui voluptatem labore assumenda amet.",
    :user => User.first(:username => 'sblock'),
    :poster => User.first(:username => 'dgusikow')
  },
]

quotes.each do |q|
  quote = Quote.new(q)
  quote.created_at = Time.now
  quote.save || raise("Error creating #{q[:body]}")
end

# Linked quote
parent = Quote.new(
  :quote_body => "Parent for linked quote.",
  :user => User.first(:username => 'ckassulk'),
  :poster => User.first(:username => 'dgusikow'),
  :created_at => Time.now
)
parent.save || raise("Unable to save parent quote")
linked = Quote.new(
  :quote_body => "In response to anther quote.",
  :user => User.first(:username => 'dgusikow'),
  :poster => User.first(:username => 'ckassulk'),
  :parent_quote => parent,
  :created_at => Time.now
)
linked.save || raise("Unable to save linked quote")

# Favourites

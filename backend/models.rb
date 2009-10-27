require 'rubygems'
require 'dm-core'

class User
  include DataMapper::Resource
  
  property :id, Serial
  property :username, String, :nullable => false
  property :firstname, String, :nullable => false
  property :surname, String, :nullable => false
  property :password, String, :nullable => false
  property :last_posted, DateTime, :nullable => true
  property :favourite_quote_id, Integer, :nullable => true

  belongs_to :favourite_quote, :model => 'Quote' #, :child_key => [ :fav]
end

class Quote
  include DataMapper::Resource

  property :id, Serial
  property :quote_body, Text
  property :user_id, Integer, :nullable => false
  property :created_at, DateTime
  property :poster_id, Integer, :nullable => false
  property :parent_quote_id, Integer, :nullable => true

  belongs_to :user
  belongs_to :poster, :model => 'User', :child_key => [ :poster_id ]

  # belongs_to :fancier, :model => 'User', :child_key => [ :favourite_quote_id ]

  # has n, :referring_quotes, :model => 'Quote', :child_key => [ :parent_quote_id ]
  has 1, :parent_quote, :model => 'Quote', :child_key => [ :parent_quote_id ]
  
  has n, :ratings
end

class Rating
  include DataMapper::Resource

  #quote_id integer not null,
  #rater varchar not null,
  property :id, Serial
  property :quote_id, Integer, :nullable => false
  property :user_id, Integer, :nullable => false
  property :rating, Integer, :nullable => false

  belongs_to :user
  belongs_to :quote
end

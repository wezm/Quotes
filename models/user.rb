class User
  include DataMapper::Resource
  
  property :id, Serial
  property :username, String, :nullable => false
  property :firstname, String, :nullable => false
  property :surname, String, :nullable => false
  property :password, String, :nullable => false
  property :email, String, :default => 'user@example.com'
  property :last_posted, DateTime, :nullable => true
  property :favourite_quote_id, Integer, :nullable => true

  belongs_to :favourite_quote, :model => 'Quote' #, :child_key => [ :fav]
end

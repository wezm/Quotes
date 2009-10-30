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

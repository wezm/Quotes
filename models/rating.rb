class Rating
  include DataMapper::Resource

  #quote_id integer not null,
  #rater varchar not null,
  property :id, Serial
  property :quote_id, Integer, :required => true
  property :user_id, Integer, :required => true
  property :rating, Integer, :required => true

  belongs_to :user
  belongs_to :quote
end

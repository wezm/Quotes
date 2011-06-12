class Quote
  include DataMapper::Resource

  property :id, Serial
  property :quote_body, Text
  property :user_id, Integer, :required => true
  property :created_at, DateTime
  property :poster_id, Integer, :required => true
  property :parent_quote_id, Integer, :required => false

  belongs_to :user
  belongs_to :poster, :model => 'User', :child_key => [ :poster_id ]

  # belongs_to :fancier, :model => 'User', :child_key => [ :favourite_quote_id ]

  # has n, :referring_quotes, :model => 'Quote', :child_key => [ :parent_quote_id ]
  has 1, :parent_quote, :model => 'Quote', :child_key => [ :parent_quote_id ]
  
  has n, :ratings
end

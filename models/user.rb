require 'dm-validations'

class User
  include DataMapper::Resource
  
  property :id, Serial
  property :username, String, :nullable => false, :unique => true
  property :firstname, String, :nullable => false
  property :surname, String, :nullable => false
  property :password_hash, String, :nullable => false
  property :salt, String, :nullable => false
  property :email, String, :default => 'user@example.com', :format => :email_address
  property :last_posted, DateTime, :nullable => true
  property :favourite_quote_id, Integer, :nullable => true

  belongs_to :favourite_quote, :model => 'Quote' #, :child_key => [ :fav]

  attr_accessor :password, :password_confirmation
  #protected equievelant? :protected => true doesn't exist in dm 0.10.0
  #protected :id, :salt
  #doesn't behave correctly, I'm not even sure why I did this.
 
  validates_present :password_confirmation, :unless => Proc.new { |t| t.password_hash }
  validates_present :password, :unless => Proc.new { |t| t.password_hash }
  validates_is_confirmed :password
 
  def password=(keyphrase)
    @password = keyphrase
    self.salt = User.random_string(10) if !self.salt
    self.hashed_password = self.digest(@password, self.salt)
  end

  def self.authenticate(username, keyphrase)
    current_user = get(:username => username)
    return nil if current_user.nil?
    User.digest(keyphrase, current_user.salt) == current_user.password_hash ? current_user : nil
  end

  protected
  
  def self.digest(keyphrase, salt)
    Digest::SHA1.hexdigest(pass+salt)
  end

  def self.random_string(len)
    chars = ("a".."z").to_a + ("A".."Z").to_a + ("0".."9").to_a
    rand_str = ""
    1.upto(len) { |i| rand_str << chars[rand(chars.size-1)] }
    return rand_str
  end

end

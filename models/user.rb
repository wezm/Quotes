require 'dm-core'
require 'dm-validations'

class User
  include DataMapper::Resource
  
  property :id, Serial
  property :username, String, :required => true, :unique => true
  property :firstname, String, :required => true
  property :surname, String, :required => true
  property :password_hash, String, :required => true
  property :salt, String, :default => ''
  property :email, String, :default => 'user@example.com', :format => :email_address
  property :last_posted, DateTime, :required => false
  property :favourite_quote_id, Integer, :required => false

  belongs_to :favourite_quote, :model => 'Quote' #, :child_key => [ :fav]

  attr_accessor :password, :password_confirmation
  #protected equievelant? :protected => true doesn't exist in dm 0.10.0
  #protected :id, :salt
  #doesn't behave correctly, I'm not even sure why I did this.
 
  validates_presence_of :password_confirmation, :unless => Proc.new { |t| t.password_hash }
  validates_presence_of :password, :unless => Proc.new { |t| t.password_hash }
  validates_confirmation_of :password
 
  def password=(keyphrase)
    @password = keyphrase
    self.salt = User.random_string(10) if self.salt.blank?
    self.password_hash = User.digest(@password, self.salt)
  end

  def self.authenticate(username, keyphrase)
    current_user = first(:username => username)
    return nil if current_user.nil?
    User.digest(keyphrase, current_user.salt) == current_user.password_hash ? current_user : nil
  end

  protected
  
  def self.digest(keyphrase, salt)
    Digest::SHA1.hexdigest(keyphrase + salt)
  end

  def self.random_string(len)
    chars = ("a".."z").to_a + ("A".."Z").to_a + ("0".."9").to_a
    rand_str = ""
    1.upto(len) { |i| rand_str << chars[rand(chars.size-1)] }
    return rand_str
  end

end

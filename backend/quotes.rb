require 'rubygems'

require 'sinatra'
require 'dm-core'
require 'json'
require 'erb'
require 'models'

configure :development do
  db = "sqlite3://#{File.dirname(__FILE__)}/test2.db"
  db = "sqlite3:///Users/wmoore/Source/Quotes/backend/test2.db"
  puts db
  DataMapper.setup(:default, db)
end

helpers do
  include Rack::Utils
  alias_method :h, :escape_html
end

get '/' do
  quotes = Quote.all(:limit => 10)
  erb :index, :locals => { :quotes => quotes }
end

get '/quotes' do
  "all quotes paginated".to_json
end

get '/users/:name/quotes' do |username|
  "Quotes for #{username}".to_json
end


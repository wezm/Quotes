require 'rubygems'

require 'sinatra'
require 'dm-core'
require 'json'
require 'erb'

configure :development do
  DataMapper.setup(:default, "sqlite3://#{File.dirname(__FILE__)}/quotes.db")
end

get '/' do
  erb :index
end

get '/quotes' do
  "all quotes paginated".to_json
end

get '/users/:name/quotes' do |username|
  "Quotes for #{username}".to_json
end


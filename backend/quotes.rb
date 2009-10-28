require 'rubygems'

require 'sinatra/base'
require 'dm-core'
require 'json'
require 'erb'
require 'mustache/sinatra'

require 'models'

module Quotes
  class App < Sinatra::Base
    register Mustache::Sinatra

    # Should be the path to your .mustache template files.
    set :views, "templates"

    # Should be the path to your .rb Mustache view files.
    # Only needed if different from the `views` setting
    set :mustaches, "views"

    # This tells Mustache where to look for the Views module,
    # under which your View classes should live. By default it's
    # the class of your app - in this case `Hurl`. That is, for an :index
    # view Mustache will expect Hurl::Views::Index by default.

    # If our Sinatra::Base subclass was instead Hurl::App,
    # we'd want to do `set :namespace, Hurl::App`
    set :namespace, Quotes

    configure :development do
      db = "sqlite3://#{File.dirname(__FILE__)}/quotes.db"
      db = "sqlite3:///Users/wmoore/Source/Quotes/backend/quotes.db"
      puts "Using db at #{db}"
      DataMapper.setup(:default, db)
    end

    helpers do
      include Rack::Utils
      alias_method :h, :escape_html
    end

    get '/' do
      # quotes = Quote.all(:limit => 10)
      @quotes = Quote.all(:order => [:created_at.desc], :limit => 10)
      @users = User.all
      mustache :index
      # erb :index, :locals => { :quotes => quotes }
    end

    get '/quotes' do
      "all quotes paginated".to_json
    end

    get '/users/:name/quotes' do |username|
      "Quotes for #{username}".to_json
    end

    get '/stats' do
      mustache :stats
    end
  end
end

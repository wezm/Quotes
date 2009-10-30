require 'rubygems'

require 'sinatra/base'
require 'dm-core'
require 'mustache/sinatra'

require 'models/models'

module Quotes
  class App < Sinatra::Base
    register Mustache::Sinatra

    set :app_file, __FILE__

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

    enable :static
    # enable :sessions

    configure :development do
      db = "sqlite3://#{File.dirname(__FILE__)}/quotes.db" # Doesn't work...
      db = "sqlite3:///Users/wmoore/Source/Quotes/quotes.db"
      puts "Using db at #{db}"
      DataMapper.setup(:default, db)
    end

    get '/' do
      @quotes = Quote.all(:order => [:created_at.desc], :limit => 10)
      @users = User.all
      mustache :index
    end

    get '/quotes' do
      "all quotes paginated"
    end

    get '/users/:name' do |username|
      @user = User.first(:username => username)
      return not_found unless @user
      @quotes = Quote.all(:user => @user, :order => [:created_at.desc])
      mustache :user
    end

    get '/stats' do
      mustache :stats
    end
  end
end

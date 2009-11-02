require 'rubygems'

require 'sinatra/base'

require 'dm-core'

require 'mustache/sinatra'
require 'rack-flash'

require 'models/user'
require 'models/quote'

module Quotes
  class App < Sinatra::Base
    enable :sessions
    use Rack::Flash
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

    configure :development do
      db = "sqlite3://#{File.dirname(__FILE__)}/quotes.db" # Doesn't work...
      db = "sqlite3:///Users/wmoore/Source/Quotes/quotes.db"
      puts "Using db at #{db}"
      DataMapper.setup(:default, db)
    end

    helpers do
      def login_required
        if session[:user]
          return true
        else
          session[:return_to] = request.fullpath
          redirect '/login'
          return false
        end
      end
    end

    before do
      @flash = flash[:message]
    end

    get '/' do
      login_required
      @quotes = Quote.all(:order => [:created_at.desc], :limit => 10)
      @users = User.all
      mustache :index
    end

    get '/quotes' do
      login_required
      "all quotes paginated"
    end

    get '/users/:name' do |username|
      login_required
      @user = User.first(:username => username)
      return not_found unless @user
      @quotes = Quote.all(:user => @user, :order => [:created_at.desc])
      mustache :user
    end

    get '/login' do
      @title = 'Login'
      @sidebar = false
      mustache :login
    end

    post '/login' do
      if user = User.authenticate(params[:username], params[:password])
        session[:user] = user.id
        redirect '/'
      else
        flash[:message] = "Invalid username or password"
        redirect '/login'
      end
    end

    get '/logout' do
      session.delete(:user)
      flash[:message] = 'You have been logged out'
      redirect '/login'
    end

  end
end

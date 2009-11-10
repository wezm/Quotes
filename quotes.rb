require 'rubygems'

require 'sinatra/base'

require 'dm-core'
require 'dm-pager'

require 'mustache/sinatra'
require 'rack-flash'

require 'yaml'
require 'pathname'

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

    configure do
      # Load config
      basepath = Pathname(__FILE__).dirname
      config_file = basepath + 'config.yml'
      unless config_file.exist?
        raise 'You need to create a config.yml, see config.yml.example.'
      end
      config = YAML.load_file(config_file)[environment]
      db = "sqlite3://#{basepath.join(config[:database]).realpath}"
      puts "Using db at #{db}"
      DataMapper.setup(:default, db.to_s)

      set :analytics_id, config[:analytics_id]
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
      @current_user = User.get(session[:user])

      # This isn't very nice but is needed because the view can't access options
      @analytics_id = options.analytics_id
    end

    get '/' do
      login_required
      @quotes = Quote.page(params[:page], :per_page => 10, :order => [:created_at.desc])
      @users = User.all
      mustache :index
    end

    get '/users/:name' do |username|
      login_required
      @user = User.first(:username => username)
      not_found unless @user
      @quotes = Quote.page(params[:page], :per_page => 20, :user => @user, :order => [:created_at.desc])
      mustache :user
    end

    get '/quote/:id' do |quote_id|
      login_required
      @quote = Quote.get(quote_id)
      not_found unless @quote
      mustache :quote
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

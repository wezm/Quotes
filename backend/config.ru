# require 'rubygems'
require 'quotes'
# require 'sinatra'
# 
# root_dir = File.dirname(__FILE__)
# 
# set :environment, ENV['RACK_ENV'].to_sym
# set :root, root_dir
# set :app_file, File.join(root_dir, 'vote_your_album.rb')
# 
# disable :run

run Quotes::App

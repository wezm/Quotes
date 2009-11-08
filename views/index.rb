require 'gravatar_helpers'
require 'quote_helpers'

module Quotes

    module Views
      
      class Index < Mustache
        include Quotes::GravatarHelpers
        include Quotes::QuoteHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def users
          @users.map do |u|
            {
              :name => u.username,
              :avatar => gravatar(u.email, 50),
              :user_path => "/users/#{u.username}"
            }
          end
        end

        def pager
          @quotes.pager.to_html('/')
        end

      end

    end

end

require 'gravatar_helpers'

module Quotes

    module Views
      
      class Index < Mustache
        include Quotes::GravatarHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def quotes
          @quotes.map do |q|
            {
              :body => q.quote_body,
              :quotee => q.user.username,
            }
          end
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

      end

    end

end

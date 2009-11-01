require 'gravatar_helpers'

module Quotes

    module Views

      class User < Mustache
        include Quotes::GravatarHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def avatar
          gravatar(@user.email, 50)
        end

        def username
          @user.username
        end

        def name
          [@user.firstname, @user.surname].join(' ')
        end

        def email
          @user.email
        end

        def quotes
          @quotes.map do |q|
            {
              :body => q.quote_body,
            }
          end
        end
      end

    end

end

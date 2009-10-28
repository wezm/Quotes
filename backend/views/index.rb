require 'gravatar_helpers'

module Quotes

    module Views
      
      class Index < Mustache
        include Quotes::GravatarHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def quotes
          @quotes.map { |q| { :body => q.quote_body, :quotee => q.user.username } }
        end

        def users
          @users.map { |u| { :name => u.username, :avatar => gravatar(u.email, 50) } }
        end

      end

    end

end

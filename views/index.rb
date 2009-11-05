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
            created = q.created_at ? { :date => q.created_at.strftime('%a %d %b %Y %I:%M %p') } : false
            {
              :body => q.quote_body,
              :quotee => q.user.username,
              :created_at => created,
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

        def pager
          @quotes.pager.to_html('/')
        end

      end

    end

end

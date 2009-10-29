module Quotes

    module Views
      
      class User < Mustache
        def username
          @user.username
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

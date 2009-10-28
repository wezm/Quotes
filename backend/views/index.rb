module Quotes

    module Views
      
      class Index < Mustache
        
        def quotes
          @quotes.map { |q| { :body => q.quote_body, :quotee => q.user.username } }
        end
        
      end

    end

end

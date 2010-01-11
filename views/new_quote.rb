module Quotes
  
  module Views
    
    class NewQuote < Mustache
      
      def title
        'Add New Quote'
      end

      def users
        @users.map do |u|
          {
            :user_id => u.id,
            :name => u.username
            
          }
        end
      end

    end
    
  end
  
end

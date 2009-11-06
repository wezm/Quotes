module Quotes
  module Views
    class Layout < Mustache
      def title
        @title || 'Quotes'
      end
      
      def flash_message
        return false unless @flash
        { :message => @flash }
      end

      def user
        return false unless @user
        { :path => "/users/#{@user.username}" }
      end
    end
  end
end

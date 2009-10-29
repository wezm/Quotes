require 'md5'

module Quotes

  module GravatarHelpers
    def gravatar(email, size = 80)
      gravatar_id = Digest::MD5.hexdigest(email.to_s.strip.downcase)
      gravatar_for_id(gravatar_id, size)
    end

    def gravatar_for_id(gid, size = 30, rating = 'r', default = 'wavatar')
      "#{gravatar_host}/avatar/#{gid}?s=#{size}&r=#{rating}&d=#{default}"
    end

    def gravatar_host
      @ssl ? 'https://secure.gravatar.com' : 'http://www.gravatar.com'
    end
  end

end

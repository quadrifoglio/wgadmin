module Data exposing (DeviceInfo, PeerInfo)


type alias DeviceInfo =
    { name : String
    , key : String
    , listenPort : Int
    , peers : List PeerInfo
    }


type alias PeerInfo =
    { key : String
    , endpoint : String
    , allowedIps : List String
    }

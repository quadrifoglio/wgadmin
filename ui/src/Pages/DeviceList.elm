module Pages.DeviceList exposing (Model, Msg, init, update, view)

import Data exposing (DeviceInfo, PeerInfo)
import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Widgets exposing (button, titleWithIcon)


type alias Model =
    { devices : List DeviceInfo
    }


type Msg
    = GotDevices (List DeviceInfo)


init : ( Model, Cmd Msg )
init =
    ( { devices =
            [ { name = "wg0"
              , key = "6NOiHdKskqkD0GtSD/NVx9OmUy6uNQeJL+FBppMGL0A="
              , listenPort = 4200
              , peers =
                    [ { key = "8K8u6KaeVDSDuycVvtEnAUNtONEo7cn6JLRna+GlnWw="
                      , endpoint = "127.0.0.1:4000"
                      , allowedIps = [ "1.1.1.1/32", "192.168.1.0/24" ]
                      }
                    ]
              }
            , { name = "wg1"
              , key = "KJJgRsk+3Gls9G+9QA87QhhB+2wyesV2GfeO3rQd+H4="
              , listenPort = 4201
              , peers =
                    [ { key = "8K8u6KaeVDSDuycVvtEnAUNtONEo7cn6JLRna+GlnWw="
                      , endpoint = "127.0.0.1:4000"
                      , allowedIps = [ "1.1.1.1/32", "192.168.1.0/24" ]
                      }
                    , { key = "8K8u6KaeVDSDuycVvtEnAUNtONEo7cn6JLRna+GlnWw="
                      , endpoint = "127.0.0.1:4000"
                      , allowedIps = [ "1.1.1.1/32", "192.168.1.0/24" ]
                      }
                    ]
              }
            , { name = "wg2"
              , key = "8K8u6KaeVDSDuycVvtEnAUNtONEo7cn6JLRna+GlnWw="
              , listenPort = 4202
              , peers =
                    [ { key = "8K8u6KaeVDSDuycVvtEnAUNtONEo7cn6JLRna+GlnWw="
                      , endpoint = "127.0.0.1:4000"
                      , allowedIps = [ "1.1.1.1/32", "192.168.1.0/24" ]
                      }
                    ]
              }
            ]
      }
    , Cmd.none
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    ( model, Cmd.none )


view : Model -> Element Msg
view model =
    column
        [ padding 10, spacing 30 ]
        [ titleWithIcon "WireGuard Devices" "fas fa-network-wired" (rgb255 255 255 255) 20
        , viewDevices model.devices
        ]


viewDevices : List DeviceInfo -> Element Msg
viewDevices devices =
    wrappedRow
        [ spacing 30 ]
        (List.map viewDevice devices
            ++ [ viewNewDevice ]
        )


viewDevice : DeviceInfo -> Element Msg
viewDevice device =
    viewDeviceBox
        (column [ spacingXY 0 10 ]
            [ column []
                [ viewDeviceName device.name
                , viewDeviceKey device.key
                ]
            , column []
                [ viewDeviceListenPort device.listenPort
                , viewDevicePeerCount device.peers
                ]
            , viewDeviceButtons device.name
            ]
        )


viewNewDevice : Element Msg
viewNewDevice =
    viewDeviceBox
        (el
            [ centerX, centerY ]
            (link
                []
                { url = "#"
                , label = text " + Add New Interface"
                }
            )
        )


viewDeviceBox : Element Msg -> Element Msg
viewDeviceBox content =
    el
        [ Background.color (rgb255 30 30 33)
        , Border.shadow { offset = ( 2, 2 ), size = 1, blur = 2, color = rgb255 0 0 0 }
        , Font.color (rgb255 216 217 218)
        , width (px 525)
        , height (px 180)
        , padding 30
        ]
        content


viewDeviceName : String -> Element Msg
viewDeviceName name =
    titleWithIcon name "fas fa-ethernet" (rgb255 255 255 255) 18


viewDeviceKey : String -> Element Msg
viewDeviceKey key =
    el [ Font.size 16, spacingXY 0 10 ] (text key)


viewDeviceListenPort : Int -> Element Msg
viewDeviceListenPort portNumber =
    el [ Font.size 14 ] (text ("Listening on UDP port " ++ String.fromInt portNumber))


viewDevicePeerCount : List PeerInfo -> Element Msg
viewDevicePeerCount peers =
    el [ Font.size 14 ] (text (String.fromInt (List.length peers) ++ " peer(s)"))


viewDeviceButtons : String -> Element Msg
viewDeviceButtons deviceName =
    button "Show Details" "#" 12

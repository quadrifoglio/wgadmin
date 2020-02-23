module Main exposing (main)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation exposing (Key)
import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Pages.DeviceList
import Tuple exposing (mapBoth)
import Url exposing (Url)


type Model
    = Error String
    | ModelDeviceList Pages.DeviceList.Model


type Msg
    = UrlRequested UrlRequest
    | UrlChanged Url
    | MsgDeviceList Pages.DeviceList.Msg


init : () -> Url -> Key -> ( Model, Cmd Msg )
init flags url key =
    Pages.DeviceList.init
        |> mapPage ModelDeviceList MsgDeviceList


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        ( MsgDeviceList subMsg, ModelDeviceList subModel ) ->
            Pages.DeviceList.update subMsg subModel
                |> mapPage ModelDeviceList MsgDeviceList

        ( _, _ ) ->
            ( Error "Invalid application state, please reload the page", Cmd.none )


mapPage : (subModel -> Model) -> (subMsg -> Msg) -> ( subModel, Cmd subMsg ) -> ( Model, Cmd Msg )
mapPage toModel toMsg modelCmdPair =
    mapBoth toModel (Cmd.map toMsg) modelCmdPair


view : Model -> Document Msg
view model =
    { title = "wgadmin"
    , body = [ Element.layout [] (viewBody model) ]
    }


viewBody : Model -> Element Msg
viewBody model =
    column
        [ Background.color (rgb255 22 23 25)
        , Font.family
            [ Font.typeface "Helvetica Neue"
            , Font.sansSerif
            , Font.external
                { name = "FontAwesome"
                , url = "/res/fontawesome/css/all.css"
                }
            ]
        , Font.color (rgb255 246 246 247)
        , width fill
        , height fill
        , padding 20
        ]
        [ viewPageContent model ]


viewPageContent : Model -> Element Msg
viewPageContent model =
    case model of
        Error err ->
            text err

        ModelDeviceList subModel ->
            Pages.DeviceList.view subModel |> Element.map MsgDeviceList


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        , onUrlRequest = UrlRequested
        , onUrlChange = UrlChanged
        }

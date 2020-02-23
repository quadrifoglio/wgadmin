module Widgets exposing (..)

import Element exposing (..)
import Element.Background as Background
import Element.Font as Font
import Html exposing (i)
import Html.Attributes exposing (class)


titleWithIcon : String -> String -> Color -> Int -> Element msg
titleWithIcon titleText icon fontColor fontSize =
    row
        [ spacingXY 5 0
        , Font.size fontSize
        , Font.color fontColor
        ]
        [ el
            [ paddingXY 0 5
            ]
            (html (i [ class icon ] []))
        , text titleText
        ]


button : String -> String -> Int -> Element msg
button btnText targetUrl fontSize =
    link
        [ paddingXY 5 8
        , Font.color (rgb255 255 255 255)
        , Font.size fontSize
        , Background.color (rgb255 31 96 196)
        ]
        { url = targetUrl, label = text btnText }

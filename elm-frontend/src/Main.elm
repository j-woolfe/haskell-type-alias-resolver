module Main exposing (..)

-- ( controlButton
-- , controlInput
-- , controlInputModifiers
-- , controlLabel
-- , controlTextArea
-- , controlTextAreaModifiers
-- , fieldBody
-- , fieldLabel
-- , horizontalFields
-- , field
-- )

import Browser
import Bulma.CDN exposing (..)
import Bulma.Columns exposing (..)
import Bulma.Elements exposing (..)
import Bulma.Form exposing (..)
import Bulma.Layout exposing (..)
import Bulma.Modifiers exposing (Color(..), Size(..))
import Html exposing (Html, main_, text, ul)
import Html.Attributes exposing (placeholder, rows, spellcheck)
import Html.Events exposing (onClick, onInput)
import Http
import Json.Decode as D exposing (Decoder, list, map2, string)
import Json.Encode as E



-- MAIN


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }



-- MODEL


type alias Model =
    { sourceCode : Source
    , targetType : Target
    , aliases : Aliases

    -- TODO Add loading state
    }


type alias Alias =
    { matched : String
    , replaced_type : String

    -- TODO Add locations
    -- TODO Add variable map if needed
    }



-- type alias Request =
--     {


type alias Aliases =
    List Alias


init : () -> ( Model, Cmd Msg )
init _ =
    let
        model =
            -- { sourceCode = "", targetType = "", aliases = [] }
            { sourceCode = test_source, targetType = "", aliases = [] }
    in
    ( model, getAliases model )



-- UPDATE


type Msg
    = GotAliases (Result Http.Error Aliases)
    | Submit
    | UpdateSource Source
    | UpdateTarget Target


type alias Source =
    String


type alias Target =
    String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotAliases result ->
            case result of
                Ok aliases ->
                    ( { model | aliases = aliases }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )

        Submit ->
            ( model, getAliases model )

        UpdateSource source ->
            ( { model | sourceCode = source }, Cmd.none )

        UpdateTarget target ->
            ( { model | targetType = target }, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Html Msg
view model =
    main_ []
        [ stylesheet
        , aliasesView model.aliases
        ]


aliasesView : Aliases -> Section Msg
aliasesView aliases =
    section NotSpaced
        []
        [ container []
            [ codeArea
            , targetTypeInput
            , columns columnsModifiers
                []
                [ column columnModifiers
                    []
                    [ content Standard [] <|
                        -- TODO handle not matches better
                        List.map (\a -> ul [] [ text a.matched ]) aliases
                    ]
                ]
            ]
        ]


codeArea : Field Msg
codeArea =
    Bulma.Form.field []
        [ controlTextArea controlTextAreaModifiers
            []
            [ placeholder "Source code"
            , rows 20
            , spellcheck False
            , onInput UpdateSource
            ]
            [ text test_source ]
        ]


targetTypeInput : Field Msg
targetTypeInput =
    horizontalFields []
        [ fieldLabel Standard
            []
            [ controlLabel [] [ text "Target type alias:" ]
            ]
        , fieldBody []
            [ field []
                [ controlInput
                    controlInputModifiers
                    []
                    [ placeholder "Int -> String", onInput UpdateTarget ]
                    []
                ]
            , field []
                [ controlButton
                    buttonModifiers
                    []
                    [ onClick <| Submit ]
                    [ text "Submit" ]
                ]
            ]
        ]



-- HTTP


getAliases : Model -> Cmd Msg
getAliases model =
    Http.post
        { url = "http://0.0.0.0:3000/api"
        , body = Http.jsonBody <| jsonPayload model
        , expect = Http.expectJson GotAliases matchesDecoder
        }


test_source : String
test_source =
    """type TestAlias = Int
type TestAliasDupe = Int
type TestAliasString = String
type TestVariable String = Maybe String
type TestList = [Char]
type TestNestedList = [[Char]]
type TestTuple = (String, Int)
type TestTupleNested = (String, (Int, Int))
type TestUnit = ()

type TestFunc = Int -> Int
type TestFuncDupe = Int -> Int
type TestFuncString = String -> String
type TestFuncTrip = Int -> String -> Int
type TestFunTuple = (Int, Int) -> String -> (String, (Int, Int))

type TestVar a = a -> a
type TestVarMixed a = a -> Int
type TestVarDiff a b = a -> b
type TestVarTuple a b = (a, a) -> b -> (b, (a, a))"""


jsonPayload : Model -> E.Value
jsonPayload model =
    E.object
        [ ( "target_type", E.string model.targetType )
        , ( "source", E.string model.sourceCode )
        ]


matchesDecoder : Decoder (List Alias)
matchesDecoder =
    D.field "matches" (list aliasDecoder)


aliasDecoder : Decoder Alias
aliasDecoder =
    map2 Alias
        (D.field "matched" string)
        (D.field "replaced_type" string)

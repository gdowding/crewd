{-# LANGUAGE CPP #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE OverloadedStrings #-}

module Main where

import Miso
import qualified Miso.Html.Element as HE
import qualified Miso.Html.Property as HP
import qualified Miso.Property as P
import qualified Miso.Svg as S
import qualified Miso.Svg.Element as SE
import qualified Miso.Svg.Property as SP



-- crewd modules and dependencies
import qualified Schedule as S

import Data.Time.Calendar
import qualified Data.Text as T

data Action = Init  -- | Add | Subtract
  deriving (Eq, Show)

crewDevent :: Component context props S.Event Action
crewDevent = component m u v
  where
    m :: S.Event
    m = S.Event "Ballard cup" "I" (fromGregorian 2026 1 1) "1"

    u :: Action -> Effect context props S.Event Action
    u = \case
      Init -> io_ (consoleLog "hello world!")
      -- Add -> S.Event "Ballard cup" "I" "2026-01-01" "1"
      -- Subtract -> S.Event "Ballard cup" "I" "2026-01-01" "1"


    v :: context -> props -> S.Event -> View context S.Event Action
    v _context _props e =
      HE.div_
      [ HP.class_ "min-h-screen bg-slate-900 text-slate-100 font-sans"
      ]
      [ HE.header_
        [
          HP.class_ "bg-slate-800 border-b border-slate-700 px-6 py-4 flex justify-between items-center sticky top-0 z-10"
        ]
        [

        ]
      , HE.main_
        [ HP.class_ "p-6 max-w-4xl mx-auto space-y-6"
        ]
        [ HE.div_
          [ HP.class_ "flex flex-col space-y-4"
          ]
          [
           eventDiv e
          ]
        ]
      ]

    eventDiv e =
      HE.div_
      [ HP.class_ "bg-slate-800 p-5 rounded-lg border border-slate-700 hover:border-slate-600 transition flex flex-col md:flex-row md:items-center justify-between gap-4"
      ]
      [ eventInfo e
      , eventCrew e
      , rsvp
      ]

    eventInfo e =
      -- event regatta, series, sponsor and date
      HE.div_
      [ HP.class_ "flex-1 space-y-1"
      ]
      [ HE.div_
        [ HP.class_ "flex items-center space-x-2"
        ]
        [
          HE.span_
          [ HP.class_ "text-xs uppercase font-bold tracking-wider text-orange-400"
          ]
          [ text $ ms $ eventName e
          ]
        , HE.span_
          [ HP.class_ "text-[10px] bg-slate-700 text-slate-300 px-2 py-0.5 rounded uppercase font-semibold"
          ]
          [ " STYC"
          ]
        ]
      , HE.h3_ [HP.class_ "text-lg font-semibold text-white"] ["Race ", text . ms $ S.race e]

      , HE.p_ [HP.class_ "text-xs text-slate-400"] [text . ms . show $ S.start_date e]
      ]


    eventCrew _e =
      let
        crew = ["George", "Colleen", "Lyon"]
      in
        HE.div_
        [ HP.class_ "md:w-1/3"
        ]
        [ HE.p_
          [
            HP.class_ "text-xs font-semibold text-slate-400 mb-1.5 flex items-center gap-1"
          ]
          [ crewIcon
          , text . ms $ unwords ["Assigned Crew", (numCrew crew), ":"]
          ]
        , HE.div_ [ HP.class_ "flex flex-wrap gap-1" ]  $ map formatCrew crew
        ]
        where
          numCrew c  = "(" ++ n ++ ")"
            where
              n = (show . length) c

          crewIcon =
            S.svg_
            [ HP.width_ "24"
            , HP.height_ "24"
            , SP.viewBox_ "0 0 24 24"
            , SP.fill_ "none"
            , SP.stroke_ "currentColor"
            , SP.strokeWidth_ "2"
            , SP.strokeLinecap_ "round"
            , SP.strokeLinejoin_ "round"
            , HP.class_ "lucide lucide-users w-3.5 h-3.5"
            , P.textProp "aria-hidden" "true"
            ]
            [
              SE.path_ [SP.d_ "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"]
            , SE.path_ [SP.d_ "M16 3.128a4 4 0 0 1 0 7.744"]
            , SE.path_ [SP.d_ "M22 21v-2a4 4 0 0 0-3-3.87"]
            , SE.circle_ [SP.cx_ "9", SP.cy_ "7", SP.r_ "4"]
            ]

          formatCrew c =
            HE.span_ [ HP.class_ "text-[11px] px-2 py-0.5 rounded bg-slate-700/80 text-slate-200"] [c]

    rsvp =
      HE.div_
      [ HP.class_ "flex md:flex-col items-center md:items-end justify-between md:justify-center border-t md:border-t-0 md:border-l border-slate-700/60 pt-3 md:pt-0 md:pl-5 gap-3"]
      [
        HE.div_  [HP.class_ "flex items-center space-x-1.5 relative"]
        [ attendingButton
        , tentativeButton
        , declineButton
        ]
      ]


      where
        makeButton title icon =
          HE.button_
          [ HP.class_ "p-2 rounded transition bg-slate-700 text-slate-400 hover:text-white"
          , HP.title_ title
          ]
          [ icon
          ]

        attendingButton = makeButton "Attending" attendingIcon

        attendingIcon =
          S.svg_
          iconAttributes
          [ SE.path_ [ SP.d_ "M21.801 10A10 10 0 1 1 17 3.335"]
          , SE.path_ [ SP.d_ "m9 11 3 3L22 4"]
          ]

        tentativeButton = makeButton "Tentative" tentativeIcon

        tentativeIcon =
          S.svg_
          iconAttributes
          [ SE.circle_ [SP.cx_ "12", SP.cy_ "12", SP.r_ "10" ]
          , SE.path_ [ SP.d_ "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"]
          , SE.path_ [ SP.d_ "M12 17h.01"]
          ]

        declineButton = makeButton "Decline" declineIcon

        declineIcon =
          S.svg_
          iconAttributes
          [ SE.path_ [ SP.d_ "M15 3h6v6" ]
          , SE.path_ [ SP.d_ "M10 14 21 3" ]
          , SE.path_ [ SP.d_ "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" ]
          ]

        iconAttributes =
          [ HP.width_ "24"
          , HP.height_ "24"
          , SP.viewBox_ "0 0 24 24"
          , SP.fill_ "none"
          , SP.stroke_ "currentColor"
          , SP.strokeWidth_ "2"
          , SP.strokeLinecap_ "round"
          , SP.strokeLinejoin_ "round"
          , HP.class_ "lucide lucide-circle-check-big w-4 h-4"
          , P.textProp "aria-hidden_" "true"
          ]


        -- tentativeIcon =

        -- </svg>
        --       </button>
        --       HE.button_ [ HP.class_ "p-2 rounded transition bg-slate-700 text-slate-400 hover:text-white" title="Tentative" ]
        --           SP.svg_ [" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" HP.class_ "lucide lucide-circle-question-mark w-4 h-4" aria-hidden="true" ]
        --               <circle cx="12" cy="12" r="10"></circle>
        --               <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
        --               <path d="M12 17h.01"></path>
        --           </svg>
        --       </button>
        --       HE.button_ [ HP.class_ "p-2 rounded transition bg-slate-700 text-slate-400 hover:text-white" title="Decline" ]
        --           SP.svg_ [" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" HP.class_ "lucide lucide-circle-x w-4 h-4" aria-hidden="true" ]
        --               <circle cx="12" cy="12" r="10"></circle>
        --               <path d="m15 9-6 6"></path>
        --               <path d="m9 9 6 6"></path>
        --           </svg>
        --       </button>
        -- ]
        --   <a href="https://www.styc.org/Ballard-Cup-Series-III" target="_blank" rel="noreferrer" HP.class_ "text-xs text-sky-400 hover:underline flex items-center gap-1">
        --       Event Page
        --       SP.svg_ [" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" HP.class_ "lucide lucide-external-link w-3 h-3" aria-hidden="true">
        --           <path d="M15 3h6v6"></path>
        --           <path d="M10 14 21 3"></path>
        --           <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
        --       </svg>
        --   </a>


    eventName e = T.unwords [S.regatta e, S.series e]


#ifdef WASM
foreign export javascript "hs_start" main :: IO ()
#endif

app :: Component context props S.Event Action
app = crewDevent

main :: IO ()
main = startApp defaultEvents app { mount = Just Init }

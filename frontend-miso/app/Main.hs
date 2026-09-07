{-# LANGUAGE CPP #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE OverloadedStrings #-}

module Main where

import Miso
import Miso.Lens
import qualified Miso.CSS as CSS
import qualified Miso.Html.Element as H
import qualified Miso.Html.Event as HE
import qualified Miso.Html.Property as HP

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
    v _context _props e = H.div_
      [ CSS.style_
        [ CSS.display "flex"
        , CSS.flexDirection "row"
        ]
      ]-- event.id
      [
        -- event name and date
        H.div_
        [
          CSS.style_
          [ CSS.display "flex"
          , CSS.flexDirection "column"
          ]
        ]
        [
          H.p_ [] [text $ ms $ eventName e, " STYC"],
          H.p_ [] ["Race ", text . ms $ S.race e],
          H.p_ [] [text . ms . show $ S.start_date e]
        ],
        -- Crew
        H.div_
        [
        ]
        [
          H.p_ [] ["Assigned Crew", "(3):"],
          H.div_
          [CSS.style_
           [ CSS.display "flex"
           , CSS.flexDirection "row"
           ]
          ]
          [ H.span_ [ CSS.style_ [ ("padding-block", spacing 0.5) , ("padding-inline", spacing 2) ] ] ["George"]
          , H.span_ [ CSS.style_ [ ("padding-block", spacing 0.5) , ("padding-inline", spacing 2) ] ] ["Coleen"]
          , H.span_ [ CSS.style_ [ ("padding-block", spacing 0.5) , ("padding-inline", spacing 2) ] ] ["Lyon"]
          ]
        ]
      ]

    eventName e = T.unwords [S.regatta e, S.series e]

    spacing x = CSS.rem $ 0.25 * x


-- global vcss variable from somewhere calc(var(--spacing)
-- .25rem
#ifdef WASM
foreign export javascript "hs_start" main :: IO ()
#endif

app :: Component context props S.Event Action
app = crewDevent

main :: IO ()
main = startApp defaultEvents app { mount = Just Init }

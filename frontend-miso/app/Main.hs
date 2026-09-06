{-# LANGUAGE CPP #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE OverloadedStrings #-}

module Main where

import Miso
import Miso.Lens
import qualified Miso.Html.Element as H
import qualified Miso.Html.Event as HE
import qualified Miso.Html.Property as HP

import qualified Schedule as S

import Data.Time.Calendar

data Action = Init  -- | Add | Subtract
  deriving (Eq, Show)

counter :: Component context props S.Event Action
counter = component m u v
  where
    m :: S.Event
    m = S.Event "Ballard cup" "I" (fromGregorian 2026 1 1) "1"

    u :: Action -> Effect context props S.Event Action
    u = \case
      Init -> io_ (consoleLog "hello world!")
      -- Add -> S.Event "Ballard cup" "I" "2026-01-01" "1"
      -- Subtract -> S.Event "Ballard cup" "I" "2026-01-01" "1"


    v :: context -> props -> S.Event -> View context S.Event Action
    v _context _props x = vfrag
      [
      -- H.button_
        -- [ HE.onClick Add, HP.id_ "add" ]
        -- [ "+" ]
      -- ,
      text (ms $ show x)
      -- , H.button_
        -- [ HE.onClick Subtract, HP.id_ "subtract" ]
        -- [ "-" ]
      ]


#ifdef WASM
foreign export javascript "hs_start" main :: IO ()
#endif

app :: Component context props S.Event Action
app = counter

main :: IO ()
main = startApp defaultEvents app { mount = Just Init }

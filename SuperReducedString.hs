import Control.Monad
import Data.List

trim :: String -> String
trim =
  join .
  map
    (\g ->
       if length g `mod` 2 == 0
         then ""
         else [head g]) .
  group

solve :: String -> String
solve str =
  if trimmed == str
    then str
    else solve trimmed
  where
    trimmed = trim str

main :: IO ()
main = interact solve'
  where
    solve' str =
      if null res
        then "Empty String"
        else res
      where
        res = solve str

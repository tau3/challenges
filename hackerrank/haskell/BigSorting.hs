import Data.List (sort)

main :: IO ()
main =
  interact $
  unlines . map show . sort . map (\s -> read s :: Integer) . tail . lines

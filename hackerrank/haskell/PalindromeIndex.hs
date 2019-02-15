import qualified Data.Vector as V

solve :: V.Vector Char -> Int
solve original = solve' original 0
  where
    solve' v i
      | V.length v < 2 = -1
      | V.last v == V.head v =
        let v' = V.init $ V.tail v
         in solve' v' (i + 1)
      | otherwise =
        if isPalindrome $ V.init v
          then V.length original - 1 - i
          else if isPalindrome $ V.tail v
                 then i
                 else -1

isPalindrome :: V.Vector Char -> Bool
isPalindrome v
  | V.length v < 2 = True
  | V.last v == V.head v =
    let v' = V.init $ V.tail v
     in isPalindrome v'
  | otherwise = False

main :: IO ()
main = interact $ unlines . map (show . solve . V.fromList) . tail . lines

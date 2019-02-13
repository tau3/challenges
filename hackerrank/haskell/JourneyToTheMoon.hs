import Control.Monad (replicateM)
import qualified Data.HashMap.Strict as M (HashMap, alter, empty, lookupDefault)
import Data.List (foldl')
import qualified Data.Set as S (Set, empty, insert, notMember)

type AdjacencyMatrix = M.HashMap Int [Int]

dfs :: Int -> AdjacencyMatrix -> S.Set Int -> S.Set Int
dfs vertice adjacencyMatrix visited =
  foldl'
    (\currentVisited adjacentVertice ->
       dfs adjacentVertice adjacencyMatrix currentVisited)
    visited'
    toVisit
  where
    visited' = S.insert vertice visited
    adjacents = M.lookupDefault [] vertice adjacencyMatrix
    toVisit = filter (`S.notMember` visited') adjacents

splitTrees :: [Int] -> AdjacencyMatrix -> [Int]
splitTrees vertices adjacencyMatrix = go [] vertices S.empty
  where
    go res (v:vs) visited
      | v `S.notMember` visited =
        let visited' = dfs v adjacencyMatrix visited
            res' = (length visited' - length visited) : res
         in go res' vs (dfs v adjacencyMatrix visited)
      | otherwise = go res vs visited
    go res [] _ = res

countPermutations :: [Int] -> Int
countPermutations sizes = go sizes 0 0
  where
    go (x:xs) s res = go xs s' res'
      where
        res' = res + s * x
        s' = s + x
    go [] _ result = result

makeAdjacencyMatrix :: [[Int]] -> AdjacencyMatrix
makeAdjacencyMatrix = foldl' insertEdge M.empty
  where
    insertEdge m [u, v] = insert (insert m v u) u v
    insertEdge _ _ = error "incorrect input"
    insert m k v = M.alter f k m
      where
        f (Just vs) = Just (v : vs)
        f Nothing = Just [v]

main :: IO ()
main = do
  [n, p] <- readInts
  edges <- replicateM p readInts
  let trees = splitTrees [0 .. n - 1] (makeAdjacencyMatrix edges)
  print $ countPermutations trees
  where
    readInts = map read . words <$> getLine

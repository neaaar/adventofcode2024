START = "S"
END = "E"
WALL = "#"

def inWorld(row:int,col:int,track:list[list[str]]):
    return row >= 0 and col >= 0 and row < len(track) and col < len(track[row])

def dfs(track:list[list[str]],start:tuple[int,int],end:tuple[int,int]):
    path:list[str] = []
    visited:set[tuple[int,int]] = set()
    queue = [start]
    while queue:
        curr = queue.pop()
        visited.add(curr)
        row,col = curr
        path.append(curr)
        if curr == end:
            break
        for displacement in [(0,-1),(0,1),(-1,0),(1,0)]:
            neigborRow, neigborCol = row + displacement[0], col + displacement[1]
            neigbor = neigborRow,neigborCol
            if neigbor in visited:
                continue
            if not inWorld(neigborRow,neigborCol,track):
                continue
            if track[neigborRow][neigborCol] == WALL:
                continue
            queue.append(neigbor)
    return path

def getCheats(path:list[str],maxCheats:int,threshold:int):
    count = 0
    for i in range(len(path)-1): # try going from one point in the path to another
        for j in range(i+1, len(path)):
            posA, posB = path[i], path[j]
            distance = abs(posA[0] - posB[0]) + abs(posA[1] - posB[1])
            if distance > maxCheats: # can't go that far
                continue
            # distance is how many steps we need to take to get to this point
            # timeSaved will be the difference in times - time it takes to get to point
            timeSaved = j - i - distance
            if timeSaved < threshold:
                continue
            count += 1
    print(count)

def getTrack(fileName:str):
    with open(fileName, 'r') as file:
        track = []
        start = 0,0
        end = 0,0
        for i,line in enumerate(file):
            curLine = line.strip()
            curRow = []
            for j,col in enumerate(curLine):
                curRow.append(col)
                if col == END:
                    end = i,j
                elif col == START:
                    start = i,j
            track.append(curRow)
        return track,start,end

def getBestCheatCounts(fileName:str,maxCheats:int,threshold:int):
    track,start,end = getTrack(fileName)
    path = dfs(track,start,end)
    getCheats(path,maxCheats,threshold)

def main():
    getBestCheatCounts("input.txt",2,100)
    getBestCheatCounts("input.txt",20,100)

if __name__ == "__main__":
    main()

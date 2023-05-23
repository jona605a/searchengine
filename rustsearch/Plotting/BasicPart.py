import matplotlib.pyplot as plt

#data in ms
idx23 =[[24,203,261,541,1013],
        [472,62615,525202,1256446,4049120]]

idx45 = [[17,288,319,622,1239,2785,9469,25957,80443,295296,1046282],
         [14,267,316,539,1225,2491,6610,13946,31753,56216,103062]]

s23 =[[0,53001,130971,703579,2241587],
        [0,6156,12624,43464,415580]]

s45 = [[0,4,6,6,11,24,60,142,290,1105,3010],
         [0,4,6,5,5,12,21,55,47,90,148]]


ticks = [0.1,1,2,5,10,20,50,100,200,400,800]
colors = ["b","g","r","c"]

def plotfig(data,indexes,type,logy=False,logx=False):
    plt.figure(figsize=(10,6))

    
    for i in range(len(data)):
        n = len(data[i])
        plt.plot(ticks[:n],data[i], label = f"Index{i+indexes[0]}",color=colors[indexes[i]-2])

    plt.xticks(ticks = ticks[:n] ,labels=ticks[:n], rotation=45)

    plt.title(f"{type} time over file sizes. Basic part")
    plt.ylabel(f"{type} time (ms)")
    plt.xlabel("Filesize (kb)")
    plt.legend()
    if logy:
        plt.yscale("log")
    if logx:
        plt.xscale("log")
    plt.savefig(f"../../LaTeX/Pictures/Results/BP{type}{indexes}")
    plt.show()


#plot search 4,5
plotfig(s45,[4,5],"Search")
#plot search 2,3
plotfig(s23,[2,3],"Search")
#plot search 2,3
plotfig(s23+s45,[2,3,4,5],"Search",True,True)
#plot indexing 4,5
plotfig(idx45,[4,5],"Indexing")
#plot indexing 2
plotfig([idx23[0]],[2],"Indexing")
#plot indexing 3
plotfig([idx23[1]],[3],"Indexing")
#plit indexing all
plotfig(idx23+idx45,[2,3,4,5],"Indexing",True,True)


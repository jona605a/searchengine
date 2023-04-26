import matplotlib.pyplot as plt
import os
import json
import numpy as np
import matplotlib as mpl

data = {}

last_filesize = None


#Load data from folder criterion
for folderName in os.listdir("../target/criterion"):
    
    if folderName.split()[0] == "searching":
        index,version = map(int,folderName.split()[2].split("_"))
        filesize = int(folderName.split()[5][:-3])
        depth = int(folderName.split()[7])

        if filesize not in data: 
            data[filesize] = {}
        if f"searching{(index,version)}" not in data[filesize]:
            data[filesize][f"searching{(index,version)}"] = {"mean": [None]*7, "lower_bound": [None]*7,"upper_bound": [None]*7}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][f"searching{(index,version)}"]["mean"][depth-1] = estimates["mean"]["point_estimate"]
        data[filesize][f"searching{(index,version)}"]["lower_bound"][depth-1] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][f"searching{(index,version)}"]["upper_bound"][depth-1] = estimates["mean"]["confidence_interval"]["upper_bound"]
    
    if folderName.split()[0] == "indexing":
        index,version = map(int,folderName.split()[2].split("_"))
        filesize = int(folderName.split()[3][:-2])
        
        if filesize not in data: 
            data[filesize] = {}
        if f"indexing{(index,version)}" not in data[filesize]:
            data[filesize][f"indexing{(index,version)}"] = {"mean": None, "lower_bound": None,"upper_bound": None}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][f"indexing{(index,version)}"]["mean"] = estimates["mean"]["point_estimate"]
        data[filesize][f"indexing{(index,version)}"]["lower_bound"] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][f"indexing{(index,version)}"]["upper_bound"] = estimates["mean"]["confidence_interval"]["upper_bound"]
    
    if folderName.split()[0] == "prefix":
        index,version = map(int,folderName.split()[2].split("_"))
        filesize = int(folderName.split()[5][:-2])
        
        if filesize not in data: 
            data[filesize] = {}
        if f"prefix{(index,version)}" not in data[filesize]:
            data[filesize][f"prefix{(index,version)}"] = {"mean": None, "lower_bound": None,"upper_bound": None}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][f"prefix{(index,version)}"]["mean"] = estimates["mean"]["point_estimate"]
        data[filesize][f"prefix{(index,version)}"]["lower_bound"] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][f"prefix{(index,version)}"]["upper_bound"] = estimates["mean"]["confidence_interval"]["upper_bound"]

    if folderName.split()[0] == "Find":
        index,version = map(int,folderName.split()[2].split("_"))
        filesize = int(folderName.split()[3][:-2])
        
        if filesize not in data: 
            data[filesize] = {}
        if f"Find{(index,version)}" not in data[filesize]:
            data[filesize][f"Find{(index,version)}"] = {"mean": None, "lower_bound": None,"upper_bound": None}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][f"Find{(index,version)}"]["mean"] = estimates["mean"]["point_estimate"]
        data[filesize][f"Find{(index,version)}"]["lower_bound"] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][f"Find{(index,version)}"]["upper_bound"] = estimates["mean"]["confidence_interval"]["upper_bound"]
        

def plot_indexing(data,indexes):
    for index in indexes:
        mean = np.array([])
        upper_bound = np.array([])
        lower_bound = np.array([])
        for filesize in sorted(data.keys()):
            if filesize == 400 and index == 9:
                continue
            mean = np.append(mean,data[filesize][f"indexing{index}"]["mean"])
            upper_bound = np.append(upper_bound,data[filesize][f"indexing{index}"]["upper_bound"])
            lower_bound = np.append(lower_bound,data[filesize][f"indexing{index}"]["lower_bound"])
        
        x = [1, 2, 5, 10, 20, 50, 100, 200]
        #x = [1, 2, 5, 10, 20, 50, 100, 200,400]
        
        plt.fill_between(x,lower_bound[:8],upper_bound[:8],label = f"index{index}")
        #plt.fill_between(x,lower_bound,upper_bound,label = f"index{index}")
           
        plt.xticks(x,["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB"])
        #plt.xticks(x,["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])
        plt.title(f"Indexing Time over filesize.")
        plt.xlabel("Filesize")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
    plt.show()
            
def plot_depth(data, indexes):
    for filesize in sorted(data.keys()):
        for index in indexes:
            plt.plot(data[filesize][f"searching{index}"]["mean"], label = f"index {index[0]}.{index[1]}")
            x = np.linspace(0,6,7)
            y1 = data[filesize][f"searching{index}"]["upper_bound"]
            y2 = data[filesize][f"searching{index}"]["lower_bound"]

            plt.fill_between(x,y1,y2)
            
        
        plt.xticks(range(0,7),labels=range(1,8))
        plt.title(f"Searching Time over depth of query filesize:{filesize} MB")
        plt.xlabel("depth of query")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
        plt.show()

def plot_filesize(data, indexes):

    depth_len = len(data[list(data.keys())[0]][f"searching{indexes[0]}"]["mean"])

    for i in range(depth_len):
        for index in indexes:
            mean = np.array([])
            upper_bound = np.array([])
            lower_bound = np.array([])
            for filesize in sorted(data.keys()):
                mean = np.append(mean,data[filesize][f"searching{index}"]["mean"][i])
                upper_bound = np.append(upper_bound,data[filesize][f"searching{index}"]["upper_bound"][i])
                lower_bound = np.append(lower_bound,data[filesize][f"searching{index}"]["lower_bound"][i])

            x = [1, 2, 5, 10, 20, 50, 100, 200, 400]
            plt.fill_between(x,lower_bound,upper_bound,label = f"index {index[0]}.{index[1]}")
                    
        plt.xticks([1, 2, 5, 10, 20, 50, 100, 200, 400],["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])
        plt.title(f"Searching Time over filesize. Depth:{i+1}")
        plt.xlabel("Filesize")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
        plt.show()

def plot_find_word(data, indexes):
    
    for index in indexes:
            mean = np.array([])
            upper_bound = np.array([])
            lower_bound = np.array([])
            
            for filesize in sorted(data.keys()):
                mean = np.append(mean,data[filesize][f"Find{index}"]["mean"])
                upper_bound = np.append(upper_bound,data[filesize][f"Find{index}"]["upper_bound"])
                lower_bound = np.append(lower_bound,data[filesize][f"Find{index}"]["lower_bound"])

            x = [1, 2, 5, 10, 20, 50, 100, 200, 400]
            plt.fill_between(x,lower_bound,upper_bound,label = f"index {index}")
                    
    plt.xticks([1, 2, 5, 10, 20, 50, 100, 200, 400],["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])
    plt.title(f"Searching Time for whole word over filesize")
    plt.xlabel("Filesize")
    plt.ylabel("Searching Time")
    plt.legend(loc='best')
    plt.show()

def plot_depth_filesize(data, indexes):
    
    number_of_files = len(data.keys())
    depth_len = len(data[list(data.keys())[0]][f"searching{indexes[0]}"]["mean"])

    fig = plt.figure(num=1, clear=True)
    ax = fig.add_subplot(1, 1, 1, projection='3d')

    (x, y) = np.meshgrid(np.linspace(1, depth_len,depth_len ), 
                        [1, 2, 5, 10, 20, 50, 100, 200, 400])
    legends = [None]*len(indexes)
    colors = ['tab:blue','tab:orange','tab:green','tab:red','tab:purple','tab:brown','tab:pink','tab:gray','tab:olive','tab:cyan']
    i= 0
    for index in indexes:
        z = np.array([])
        filesizes = []

        for filesize in data.keys():
            filesizes.append(filesize)
            z = np.append(z,data[filesize][f"searching{index}"]["mean"])
        
        z = z.reshape((number_of_files,depth_len))
        z = z[np.argsort(filesizes),: ]

        ax.plot_surface(x, y, z, alpha=0.4, label = f"{index[0]}.{index[1]}" , color = colors[i])
        legends[i] = mpl.lines.Line2D([0],[0], linestyle="none", marker = 'o', c = colors[i])
        i += 1
    
    ax.set(xlabel='Depth',
            ylabel='Filesize',
            zlabel='Searching Time', 
            title=f'{indexes} Searching Time',
            )
    ax.set_xticklabels(range(1,8))
    #ax.set_yscale('log',base=2)
    ax.set_yticks([1, 2, 5, 10, 20, 50, 100, 200, 400])
    ax.legend(legends, indexes)
    ax.set_yticklabels(["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])

    plt.show()

def plot_prefixsearch(data,indexes):
    for index in indexes:
        mean = np.array([])
        upper_bound = np.array([])
        lower_bound = np.array([])
        
        for filesize in sorted(data.keys()):
            mean = np.append(mean,data[filesize][f"prefix{index}"]["mean"])
            upper_bound = np.append(upper_bound,data[filesize][f"prefix{index}"]["upper_bound"])
            lower_bound = np.append(lower_bound,data[filesize][f"prefix{index}"]["lower_bound"])
        
        x = [1, 2, 5, 10, 20, 50, 100, 200,400]
        
        plt.fill_between(x,lower_bound,upper_bound,label = f"index{index}")
           
        plt.xticks(x,["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])
        plt.title("Prefix search time over filesize")
        plt.xlabel("Filesize")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
    plt.show()

booleanIndexes = [(7,0),(8,0),(8,1),(8,2),(8,3),(8,4)]

plot_indexing(data,[(7,0),(8,0),(9,0),(9,1)])
#plot_depth(data,booleanIndexes)
#plot_filesize(data,booleanIndexes)
plot_find_word(data,[(8,0),(9,0),(9,1)])
#plot_depth_filesize(data,booleanIndexes)
plot_prefixsearch(data,[(9,0),(9,1)])
            





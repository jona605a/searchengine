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
        if (index,version) not in data[filesize]:
            data[filesize][(index,version)] = {"mean": [None]*7, "lower_bound": [None]*7,"upper_bound": [None]*7}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][(index,version)]["mean"][depth-1] = estimates["mean"]["point_estimate"]
        data[filesize][(index,version)]["lower_bound"][depth-1] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][(index,version)]["upper_bound"][depth-1] = estimates["mean"]["confidence_interval"]["upper_bound"]
    
    if folderName.split()[0] == "indexing":
        index = int(folderName.split()[2])
        filesize = int(folderName.split()[3][:-2])
        
        if filesize not in data: 
            data[filesize] = {}
        if index not in data[filesize]:
            data[filesize][index] = {"mean": None, "lower_bound": None,"upper_bound": None}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][index]["mean"] = estimates["mean"]["point_estimate"]
        data[filesize][index]["lower_bound"] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][index]["upper_bound"] = estimates["mean"]["confidence_interval"]["upper_bound"]
    
    if folderName.split()[0] == "prefix":
        index = folderName.split()[2]
        filesize = int(folderName.split()[5][:-2])
        
        if filesize not in data: 
            data[filesize] = {}
        if index not in data[filesize]:
            data[filesize][index] = {"mean": None, "lower_bound": None,"upper_bound": None}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][index]["mean"] = estimates["mean"]["point_estimate"]
        data[filesize][index]["lower_bound"] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][index]["upper_bound"] = estimates["mean"]["confidence_interval"]["upper_bound"]

    if folderName.split()[0] == "Find":
        index = folderName.split()[2]
        filesize = int(folderName.split()[3][:-2])
        
        if filesize not in data: 
            data[filesize] = {}
        if f"Find{index}" not in data[filesize]:
            data[filesize][f"Find{index}"] = {"mean": None, "lower_bound": None,"upper_bound": None}
        
        f = open(f"../target/criterion/{folderName}/new/estimates.json")
        estimates = json.load(f)

        data[filesize][f"Find{index}"]["mean"] = estimates["mean"]["point_estimate"]
        data[filesize][f"Find{index}"]["lower_bound"] = estimates["mean"]["confidence_interval"]["lower_bound"]
        data[filesize][f"Find{index}"]["upper_bound"] = estimates["mean"]["confidence_interval"]["upper_bound"]
        
booleanIndexes = [(7,0),(8,0),(8,1),(8,2),(8,3),(8,4)]

def plot_indexing(data,indexes):
    for index in indexes:
        mean = np.array([])
        upper_bound = np.array([])
        lower_bound = np.array([])
        for filesize in sorted(data.keys()):
            if filesize == 400 and index == 9:
                continue
            mean = np.append(mean,data[filesize][index]["mean"])
            upper_bound = np.append(upper_bound,data[filesize][index]["upper_bound"])
            lower_bound = np.append(lower_bound,data[filesize][index]["lower_bound"])
        
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

#plot_indexing(data,[7,8,9])
            
def plot_depth(data, indexes):
    
    for filesize in sorted(data.keys()):
        for index in indexes:
            plt.plot(data[filesize][index]["mean"], label = f"index {index[0]}.{index[1]}")
            x = np.linspace(0,6,7)
            y1 = data[filesize][index]["upper_bound"]
            y2 = data[filesize][index]["lower_bound"]
            plt.fill_between(x,y1,y2)
            
        
        plt.xticks(range(0,7),labels=range(1,8))
        plt.title(f"Searching Time over depth of query filesize:{filesize} MB")
        plt.xlabel("depth of query")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
        plt.show()

#plot_depth(data,booleanIndexes)

def plot_filesize(data, indexes):

    depth_len = len(data[list(data.keys())[0]][indexes[0]]["mean"])

    for i in range(depth_len):
        for index in indexes:
            mean = np.array([])
            upper_bound = np.array([])
            lower_bound = np.array([])
            for filesize in sorted(data.keys()):
                mean = np.append(mean,data[filesize][index]["mean"][i])
                upper_bound = np.append(upper_bound,data[filesize][index]["upper_bound"][i])
                lower_bound = np.append(lower_bound,data[filesize][index]["lower_bound"][i])

            x = [1, 2, 5, 10, 20, 50, 100, 200, 400]
            plt.fill_between(x,lower_bound,upper_bound,label = f"index {index[0]}.{index[1]}")
                    
        plt.xticks([1, 2, 5, 10, 20, 50, 100, 200, 400],["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])
        plt.title(f"Searching Time over filesize. Depth:{i+1}")
        plt.xlabel("Filesize")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
        plt.show()


#plot_filesize(data,booleanIndexes)

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

plot_find_word(data,[8,9])

def plot_depth_filesize(data, indexes):
    
    number_of_files = len(data.keys())
    depth_len = len(data[list(data.keys())[0]][indexes[0]]["mean"])

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
            z = np.append(z,data[filesize][index]["mean"])
        
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
    

#plot_depth_filesize(data,booleanIndexes)

def plot_prefixsearch(data,indexes):
    for index in indexes:
        mean = np.array([])
        upper_bound = np.array([])
        lower_bound = np.array([])
        
        for filesize in sorted(data.keys()):
            mean = np.append(mean,data[filesize][index]["mean"])
            upper_bound = np.append(upper_bound,data[filesize][index]["upper_bound"])
            lower_bound = np.append(lower_bound,data[filesize][index]["lower_bound"])
        
        x = [1, 2, 5, 10, 20, 50, 100, 200,400]
        
        plt.fill_between(x,lower_bound,upper_bound,label = f"index{index}")
           
        plt.xticks(x,["1MB", "2MB", "5MB", "10MB", "20MB", "50MB", "100MB", "200MB","400MB"])
        plt.title("Prefix search time over filesize")
        plt.xlabel("Filesize")
        plt.ylabel("Searching Time")
        plt.legend(loc='best')
    plt.show()

#plot_prefixsearch(data,["9_1"])
            





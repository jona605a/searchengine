import pandas as pd
import matplotlib.pyplot as plt
import math

data = pd.read_csv("articlelist_length.csv",header=None)
file_sizes = data.iloc[:,-1]
articlesprfile= data.iloc[:,-2]
data = data.iloc[:,:-2]

for i in range(len(file_sizes)):
    print(data.iloc[i,articlesprfile.iloc[i]-5:articlesprfile.iloc[i]+1])
    plt.title(f"Lengths of Article list for filesize{file_sizes[i]}")
    plt.ylabel("Number of Words with article list of given lenght")
    plt.xlabel("Lengths")
    plt.yscale("log")
    plt.xticks(range(0,articlesprfile.iloc[i],math.ceil(articlesprfile.iloc[i]/12)),range(1,articlesprfile.iloc[i]+1,math.ceil(articlesprfile.iloc[i]/12)))
    plt.bar(range(articlesprfile.iloc[i]),data.iloc[i,:articlesprfile.iloc[i]], )
    plt.savefig(f"../../LaTeX/Pictures/Results/ArticleLengthg{file_sizes[i]}")
    plt.show()

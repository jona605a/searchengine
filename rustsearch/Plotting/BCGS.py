import pandas as pd
import matplotlib.pyplot as plt


def BCGSplot(promts):

    for promt in promts:

        dataBC = pd.read_csv(f"Plotting/BCGScounting/BC{promt}.csv",header=None)
        dataGS = pd.read_csv(f"Plotting/BCGScounting/GS{promt}.csv",header=None)
        
        plt.title(f"{promt}")
        plt.xticks(range(len([*promt])),[*promt])
        plt.hist([dataBC[0],dataGS[0]],bins=len([*promt]),label=['Bad character rule', 'Good suffix rule'])
        plt.show()

BCGSplot(["state_enterprises_will_be_privatized","a_few_months_earlier_to","left_her_husband_upon_reuniting","two_wood_boxes_joined_together","with_practical_experience_of_dissection"])


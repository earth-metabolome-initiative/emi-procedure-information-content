# Information Content Approaches

| Name          | Paper Link                                                                 | Data Structure | Formula | Complexity | 
|---------------|----------------------------------------------------------------------------|----------------|---------|------------|
| Resnik (1999) | [Using Information Content to Evaluate Semantic Similarity in a Taxonomy](https://arxiv.org/pdf/cmp-lg/9511007) | Tree / DAG     | $\textbf{Information Content:} \quad IC(c) = - \log p(c)$ <br> $\textbf{Word-Level Similarity:} \quad \text{sim}(w_1, w_2) = \max_{c_1 \in s(w_1) c_2 \in s(w_2)} \text{sim}(c_1, c_2)$ <br> $\textbf{Concept-Level Similarity:} \quad \text{sim}(c_1, c_2) = \max_{c \in S(c_1, c_2)} \big[- \log p(c)\big]$|  |
| Wang (2007)   | [A new method to measure semantic similarity of GO terms](https://academic.oup.com/bioinformatics/article/23/10/1274/197095?login=false) | DAG |  |  |

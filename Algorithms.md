# Information Content Approaches
# Information Content Approaches

| Name          | Paper Link                                                                 | Data Structure | Formula | Complexity | 
|---------------|----------------------------------------------------------------------------|----------------|---------|------------|
| Resnik (1999) | [Semantic similarity in a taxonomy](https://www.aclweb.org/anthology/J99-1004.pdf) | Tree / DAG     |  |  |
| Wang (2007)   | [A new method to measure semantic similarity of GO terms](https://academic.oup.com/bioinformatics/article/23/10/1274/197095) | DAG | $$S_A(t) =\begin{cases}\mathcal{S}_A(A) = 1\\\mathcal{S}_A(t) = \max\{\mathcal{w}_e * \mathcal{S}A(t')\| t'{ \in \text{childrenof}(t)}\}\text{if } t \neq A\end{cases}$$ | O(V + E) |

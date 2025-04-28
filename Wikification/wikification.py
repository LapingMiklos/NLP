import codecs
import json
import math
import re
from typing import Callable
import pandas as pd
import ast

N = 1227183

RE_WORD = r'\w+'
re_word = re.compile(RE_WORD, re.U)

def tokenize(text: str, n: int = 5) -> list[str]:
    one_grams = [x.lower() for x in re.findall(re_word, text)]
    ngrams = [one_grams]
    for i in range(1, n):
        ngrams.append([f'{x} {y}' for x, y in zip(ngrams[-1], one_grams[i:])])

    return [item for sublist in ngrams for item in sublist]

class KeyWordExtractor:
    def extract(self, text: str) -> list[str]:
        terms = self.tokenizer(text)
        terms_with_scores = [(term, self.score(term, terms)) for term in set(terms)]
        terms_with_scores = sorted([(term, score) for term, score in terms_with_scores if score > 0], key=lambda x: x[1], reverse=True)

        extraction_count = int(len(terms_with_scores) * self.kw_ratio)
        return [term for term, _ in terms_with_scores[:extraction_count]]

class TfIdfKeyWordExtractor(KeyWordExtractor):
    def __init__(self, df: dict[str, int], N: int, tokenizer: Callable[[str], list[str]], kw_ratio: float = 0.06):
        self.idf = { term:math.log(N / df) for term, df in df.items() }
        self.tokenizer = tokenizer
        self.kw_ratio = kw_ratio

    def extract(self, text: str) -> list[str]:
        terms = self.tokenizer(text)
        terms_with_scores = [(term, self.score(term, terms)) for term in set(terms)]
        terms_with_scores = sorted([(term, score) for term, score in terms_with_scores if score > 0], key=lambda x: x[1], reverse=True)

        extraction_count = int(len(terms_with_scores) * self.kw_ratio)
        return [term for term, _ in terms_with_scores[:extraction_count]]
    
    def score(self, term: str, terms: list[str]) -> float:
        return len([0 for t in terms if t == term]) * self.idf.get(term, 0)

class KeyphrasenessKeyWordExtractor(KeyWordExtractor):
    def __init__(self, df: dict[str, int], kf: dict[str, int], tokenizer: Callable[[str], list[str]], kw_ratio: float = 0.06):
        self.df = df
        self.kf = kf
        self.kw_ratio = kw_ratio
        self.tokenizer = tokenizer
    
    def extract(self, text: str) -> list[str]:
        terms = self.tokenizer(text)
        terms_with_scores = [(term, self.score(term)) for term in set(terms)]
        terms_with_scores = sorted([(term, score) for term, score in terms_with_scores if score > 0], key=lambda x: x[1], reverse=True)

        extraction_count = int(len(terms_with_scores) * self.kw_ratio)
        return [term for term, _ in terms_with_scores[:extraction_count]]
    
    def score(self, term: str) -> float:
        kf = self.kf.get(term)
        df = self.df.get(term)
        return 0 if kf is None or df is None else kf / df

def precision(pred: list[str], actual: list[str]) -> float:
    return len([0 for word in pred if word in actual]) / len(pred)

def recall(pred: list[str], actual: list[str]) -> float:
    return len([0 for word in pred if word in actual]) / len(actual)

def metrics(pred: list[str], actual: list[str]) -> tuple[float, float, float]:
    prec = precision(pred, actual)
    rec = recall(pred, actual)

    return prec, rec, 0 if prec == 0 or rec == 0 else (2 / (1 / prec + 1 / rec))

def wikify(text: str, kws: list[str]) -> str:
    for word in kws:
        pattern = re.compile(r'\b' + re.escape(word) + r'\b', flags=re.IGNORECASE)

        match = pattern.search(text)
        if match:
            start, end = match.span()
            text = text[:start] + '[[' + text[start:end] + ']]' + text[end:]
    
    return text

if __name__ == '__main__':
    print(tokenize("a b c d e f g h"))
    df_file = codecs.open('df.json', 'r', encoding='utf8')
    df = json.load(df_file)
    df_file.close()

    kf_file = codecs.open('kf.json', 'r', encoding='utf8')
    kf = json.load(kf_file)
    kf_file.close()

    kf = { k.lower():v for k,v in kf.items() }

    # kw_extractor = TfIdfKeyWordExtractor(df, N, tokenize, 0.1)
    kw_extractor = KeyphrasenessKeyWordExtractor(df, kf, tokenize, 0.09)
    
    data = pd.read_csv('test.csv')

    print(data.head())
    
    data['kws'] = data['kws'].apply(lambda kws: ast.literal_eval(kws))
    data['kws_pred'] = data['abs'].apply(lambda text: kw_extractor.extract(text))
    data['kws_count'] = data['kws'].apply(lambda kws: len(kws))
    data['pred_kws_count'] = data['kws_pred'].apply(lambda kws: len(kws))
    data['wikified'] = data.apply(
        lambda row: wikify(row['abs'], row['kws_pred']),
        axis=1
    )

    prec = 0
    rec = 0
    f_measure = 0
    for _, document in data.iterrows():
        p, r, f = metrics(document['kws_pred'], document['kws'])
        prec += p
        rec += r
        f_measure += f
    
    prec /= len(data)
    rec /= len(data)
    f_measure /= len(data)
    print(f'Precision: {prec}')
    print(f'Recall: {rec}')
    print(f'F-measure: {f_measure}')

    print(data.head())

    data.to_csv('pred.csv')
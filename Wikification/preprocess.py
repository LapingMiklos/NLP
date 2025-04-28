import codecs
import json
from xml.sax.handler import ContentHandler
import xml.sax
import re
import time

LENGTH_THRESHOLD = 1024
STEP_PRINT = 1000

# Num. of docs =  1,227,183 (>=1024)

RE_WORD = r'\w+'
re_word = re.compile(RE_WORD, re.U)

RE_KW = r'\[\[[\w |]+\]\]'
re_kw = re.compile(RE_KW, re.U)

def tokenize(text: str, n: int = 5) -> list[str]:
    one_grams = [x.lower() for x in re.findall(re_word, text)]
    ngrams = [one_grams]
    for i in range(1, n):
        ngrams.append([f'{x} {y}' for x, y in zip(ngrams[-1], one_grams[i:])])

    return [item for sublist in ngrams for item in sublist]

def get_kw(word: str) -> str:
    if '|' in word:
        return word[2:word.find('|')].lower()
    
    return word[2:-2].lower()

def tokenize_kw(text: str) -> list[str]:
    return [get_kw(x) for x in re.findall(re_kw, text)]


class WikiHandler(ContentHandler):
    def __init__(self, given_titles: list[str]):
        self.inside = False
        self.text = ''
        self.counter = 0
        self.df = {}
        self.kf = {}
        self.docs = 0

        self.given_titles = set(given_titles)

        self.title = ''
        self.inside_title = False
        self.titles = []

    def startElement(self, name, attrs):
        if name == 'text':
            self.inside = True
            self.text = ''
            self.counter += 1
            if self.counter % STEP_PRINT == 0:
                print(self.counter)
        if name == 'title':
            self.inside_title = True
            self.title = ''

    def endElement(self, name):
        if name == 'text':
            if len(self.text) >= LENGTH_THRESHOLD:
                self.docs += 1
                tokens = tokenize(self.text)
                unique_relavant_tokens = [token for token in set(tokens) if token in self.given_titles]
                for t in unique_relavant_tokens:
                    self.df[t] = self.df.get(t, 0) + 1
        
                self.titles.append(self.title.lower())

                tokens = tokenize_kw(self.text)
                for t in tokens:
                    self.kf[t] = self.kf.get(t, 0) + 1
                
            self.inside = False
        if name == 'title':
            self.inside_title = False

    def characters(self, content):
        if self.inside:
            self.text += content
        if self.inside_title:
            self.title += content

if __name__ == '__main__':
    fname = 'enwiki-20061130-pages-articles.xml'

    titles_file = codecs.open('titles.json', 'r', encoding='utf8')
    titles = json.load(titles_file)
    titles_file.close()

    parser = xml.sax.make_parser()
    handler = WikiHandler(titles)
    parser.setContentHandler(handler)

    start = time.time()
    print("Started Parsing")
    parser.parse(codecs.open(fname, 'r', encoding='utf8'))
    end = time.time()
    print(f'Time elapsed: {end - start}')

    df_file = codecs.open('df.json', 'w', encoding='utf8')
    json.dump(handler.df, df_file, indent=2)
    df_file.close()

    titles_file = codecs.open('titles.json', 'w', encoding='utf8')
    json.dump(handler.titles, titles_file, indent=2)
    titles_file.close()

    kf_file = codecs.open('kf.json', 'w', encoding='utf8')
    json.dump(handler.kf, kf_file, indent=2)
    kf_file.close()

    print(f'Docs processed: {handler.docs}')

class Greet(object):
    def __init__(self, noun):
        self._noun = noun

    def greet(self):
        return "Hello %s!" % self._noun

    def saluer(self):
        return "Bonjour %s!" % self._noun

    def japanese(self):
        return "Konichiwa %s!" % self._noun

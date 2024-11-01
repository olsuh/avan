import requests

url = "https://avan.market/v1/api/users/catalog"

params = {"app_id":"252490","currency":"2","page":"30"}
cookies = {
    "__cflb": "02DiuGa87D2dctS5ktLNzpvSQgAYtqmuqyjmBM3G61zpk",
    "cf_clearance": "1c2kErYJ2vu_OllMq87N76gjh9pGa047C7L84q7kZF8-1730367715-1.2.1.1-Ruubk9AC4EQxwNWQ6HYHvOWblNty.GNb7GRFP4c.M4m.e1EncR.RvXHbcZeII2v0CBL7xMjxTLrTOSEU90uzzeytyQXmJWoP9lVj1egMdg.dNanefGd8Jp5YhM10Z4U_sP.2zuJDCyf35j4.Af9b0ntYUWceajkHUm_wnhHxVwLiVYYjFo.UOB.2DaZL82U8mVmKmA0cPuZ5hf3nSBaQ.U.MSAz4afXkaxLUiY.OITs4xfetdiadbf.kTmj5rspnw4MLV1SZsZ30qa.OUadeaZ6IImBC2tY0ccrhUI4bFwuhHDe4IzHdq8z9IvOeFW9lv6gfOB6jaUyu7F0P0HCDSkiYzZb9z6Twl7pBuXOMoOg",
    "_ym_uid": "1730026173609601634",
    "_ym_d": "1730026173",
    "_ym_isad": "1",
    "_ga_8NDB4PRW90": "GS1.1.1730368255.9.1.1730368309.6.0.0",
    "_ga": "GA1.1.2017288820.1730026173",
    "adtech_uid": "6fc1c5ba-1995-40bf-b6c4-931a9ac71e21%3Aavan.market",
    "top100_id": "t1.7726955.303295770.1730026174615",
    "t3_sid_7726955": "s1.412506720.1730368256333.1730368328857.7.12",
    "i18next": "ru",
    "_ym_visorc": "w"
}
headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0",
"Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8",
}
resp = requests.get(url, headers=headers, cookies=cookies, params=params)

print(resp.text)

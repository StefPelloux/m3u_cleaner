
# M3U Cleaner

M3U Cleaner est un petit outil en Rust pour nettoyer les fichiers M3U en permettant de supprimer des groupes de chaînes spécifiques. L'idée et de passer ensuite sur une interface graphique
Premier test en Rust ..



## Fonctionnalités

- Lecture des fichiers M3U
- Affichage des groupes de chaînes
- Suppression des groupes sélectionnés
- Interface graphique simple pour une utilisation facile

## Prérequis

- Rust (version stable recommandée)
- Cargo (gestionnaire de paquets de Rust)

## Installation

1. Clonez ce dépôt :

```sh
git clone 
cd m3u-cleaner
```

2. Ajoutez les dépendances nécessaires dans le fichier `Cargo.toml` :

```toml
[dependencies]
regex = "1.5"
egui = "0.15"
eframe = "0.15"
```

## Utilisation

### Exécution du programme

Pour exécuter le programme, utilisez la commande suivante :

```sh
cargo run
```

### Interface graphique

1. **Charger les groupes** :
    - Cliquez sur le bouton "Charger les groupes" pour charger les groupes de chaînes à partir du fichier M3U spécifié.

2. **Sélectionner un groupe** :
    - Les groupes disponibles s'affichent avec des boutons radio pour la sélection.

3. **Supprimer un groupe** :
    - Sélectionnez un groupe et cliquez sur "Supprimer le groupe sélectionné" pour le supprimer du fichier M3U.

4. **Spécifier le chemin du fichier M3U** :
    - Entrez le chemin du fichier M3U dans le champ prévu à cet effet.

5. **Quitter l'application** :
    - Cliquez sur "Quitter" pour fermer l'application.

### Exemple de fichier M3U

Voici un exemple de contenu de fichier M3U :

```m3u
#EXTM3U
#EXTINF:-1 group-title="Group 1", Channel 1
http://example.com/stream1
#EXTINF:-1 group-title="Group 2", Channel 2
http://example.com/stream2
#EXTINF:-1 group-title="Group 1", Channel 3
http://example.com/stream3
```

## Exemple sortie

Groupes trouv├®s :
1: EU | FR  PRIMEZIC AA
2: EU | FRANCE ZIC Test 1
3: EU | FRANCE ZIC Test ABA
4: EU | FRANCE ZIC Test 3
5: EU | FRANCE ZIC Test BOB
6: VIP | USA ZIC 1
Entrez le num├®ro du groupe ├á supprimer (0 pour sortir) : 



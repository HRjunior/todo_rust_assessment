README for rust todo evaluation

Comment utiliser le todo :

1- En faisant un cargo build puis "Cargo run -- {CMD} 'parameter' "

2- en ajoutant le dossier '/target/release' au PATH : https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/	Puis en exécutant directement "rusty-journal.exe {CMD} 'parameter' "

[CMD]
CMD est la commande que vous voulez effectuer, il y a au choix :
-ajouter "task_name"
-finie "task_id"		#L'ID est visible en listant les taches, de plus, marquer une tache comme finie la supprimera
-lister
-help

[OPTIONS]

Il est possible en utilisant -j ou --journal-file de spécifier un fichier de journal différent



#__HRJ__
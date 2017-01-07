//
// Created by mistlight on 12/22/2016.
//

#include "MainWindow.h"

MainWindow::MainWindow(Directory* directoryModel, Audiobook* audiobookModel, Core::ConcretePlayer* player, QWidget *parent) :
    QMainWindow(parent), ui(new Ui::MainWindow()) {
    ui->setupUi( this );

    // we will need this reference so FileList can make direct reference to it
    this->concretePlayer = player;

    // a hack so the menu shows up on mac
    ui->menubar->setNativeMenuBar(false);

    // set the model
    this->directoryModel = directoryModel;
    this->audiobookModel = audiobookModel;
    this->fileDisplayModel = new FileDisplayModel(this);

    // initialize the settings form
    this->settingsForm = new SettingsForm(this->directoryModel);

    this->setup();
}

void MainWindow::setup() {
    this->setWindowTitle("Nodoka");

    connect(this->ui->actionExit, &QAction::triggered, this, &MainWindow::performExit);
    connect(this->ui->actionSettings, &QAction::triggered, this, &MainWindow::performSettings);

    // set up the audobook view
    this->ui->audiobookView->setModel(this->audiobookModel);

    // connect the audiobook view events to the file selector view
    auto audiobookModel = this->audiobookModel;
    connect(this->ui->audiobookView->selectionModel(), &QItemSelectionModel::selectionChanged,
            [this, audiobookModel] (const QItemSelection &selected, const QItemSelection &deselected) {
                if(selected.indexes().size() > 0) {
                    auto modelIndex = selected.indexes().first();
                    auto record = audiobookModel->record(modelIndex.row());
                    int audiobookId = record.value("id").toInt();

                    this->fileDisplayModel->setSelectedAudiobook(audiobookId);
                }
            });

    this->ui->fileView->setModel(this->fileDisplayModel);


    // connect file selector view to concrete player
    connect(this->ui->fileView, &QListView::doubleClicked,
            [this] (const QModelIndex &index) {
                QSqlTableModel* model = (QSqlTableModel *) this->ui->fileView->model();
                auto row = index.row();

                auto currentRecord = model->record(row);

                auto path = currentRecord.value("full_path").toString();
                this->concretePlayer->loadMedia(path);
                this->concretePlayer->play();
    });

    // prevent editing of audiobook and file list view
    this->ui->audiobookView->setEditTriggers(QAbstractItemView::NoEditTriggers);
    this->ui->fileView->setEditTriggers(QAbstractItemView::NoEditTriggers);
}


MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::performExit() {
    this->close();
}

void MainWindow::performSettings() {
    this->settingsForm->setWindowModality(Qt::WindowModality::ApplicationModal);
    this->settingsForm->show();
}


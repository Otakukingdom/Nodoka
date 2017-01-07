//
// Created by mistlight on 12/22/2016.
//

#include "MainWindow.h"

MainWindow::MainWindow(Directory* directoryModel, Audiobook* audiobookModel, QWidget *parent) :
    QMainWindow(parent), ui(new Ui::MainWindow()) {
    ui->setupUi( this );

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


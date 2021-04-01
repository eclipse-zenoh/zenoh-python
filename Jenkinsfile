pipeline {
  agent { label 'MacMini' }
  options { skipDefaultCheckout() }
  parameters {
    gitParameter(name: 'GIT_TAG',
                 type: 'PT_BRANCH_TAG',
                 description: 'The Git tag to checkout. If not specified "master" will be checkout.',
                 defaultValue: 'master')
    booleanParam(name: 'PUBLISH_RESULTS',
                 description: 'Publish the resulting wheels to Eclipse download and to pypi.org (if not a branch)',
                 defaultValue: false)
    booleanParam(name: 'BUILD_MACOSX',
                 description: 'Build wheels for macosx_10_7_x86_64.',
                 defaultValue: true)
    booleanParam(name: 'BUILD_LINUX64',
                 description: 'Build wheels with manylinux2010-x86-64.',
                 defaultValue: true)
    booleanParam(name: 'BUILD_LINUX32',
                 description: 'Build wheels with manylinux2010-i686.',
                 defaultValue: true)
    booleanParam(name: 'BUILD_AARCH64',
                 description: 'Build wheels with manylinux2014-aarch64.',
                 defaultValue: true)
  }
  environment {
      LABEL = get_label()
      DOWNLOAD_DIR="/home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${LABEL}"
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        deleteDir()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.GIT_TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-python.git']]
                ])
      }
    }

    stage('MacOS wheels') {
      when { expression { return params.BUILD_MACOSX }}
      steps {
        sh '''
        set +x
        . ~/.zshenv
        env
        maturin build --release $MATURIN_PYTHONS_OPT
        '''
      }
    }

    stage('Manylinux2010-x64 wheels') {
      when { expression { return params.BUILD_LINUX64 }}
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/zenoh-dev-manylinux2010-x86_64-gnu maturin build --release --manylinux 2010
        '''
      }
    }

    stage('Manylinux2010-i686 wheels') {
      when { expression { return params.BUILD_LINUX32 }}
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/zenoh-dev-manylinux2010-i686-gnu maturin build --release --manylinux 2010
        '''
      }
    }

    stage('Manylinux2014-aarch64 wheels') {
      when { expression { return params.BUILD_AARCH64 }}
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir adlinktech/zenoh-dev-manylinux2014-aarch64-gnu maturin build --release --manylinux 2014
        '''
      }
    }

    stage('Deploy to download.eclipse.org') {
      when { expression { return params.PUBLISH_RESULTS }}
      steps {
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
            if [[ ${GIT_TAG} == origin/* ]]; then
              ssh genie.zenoh@projects-storage.eclipse.org rm -fr ${DOWNLOAD_DIR}
            fi
            ssh genie.zenoh@projects-storage.eclipse.org mkdir -p ${DOWNLOAD_DIR}
            scp target/wheels/*.tar.gz genie.zenoh@projects-storage.eclipse.org:${DOWNLOAD_DIR}/
            find target -name "*.whl" | xargs -J FILES scp FILES genie.zenoh@projects-storage.eclipse.org:${DOWNLOAD_DIR}/
          '''
        }
      }
    }

    stage('Deploy on pypi.org') {
      when { expression { return params.PUBLISH_RESULTS && !env.GIT_TAG.startsWith('origin/') }}
      steps {
        sh '''
          python3 -m twine upload --repository eclipse-zenoh `find target -name "*.whl"` target/wheels/*.tar.gz
        '''
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'target/**/*.whl, target/wheels/*.tar.gz', fingerprint: true
    }
  }
}

def get_label() {
    return env.GIT_TAG.startsWith('origin/') ? env.GIT_TAG.minus('origin/') : env.GIT_TAG
}
